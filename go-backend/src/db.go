package main

import (
	"context"
	"fmt"
	"os"

	// "github.com/jackc/pgx/v5"

	"github.com/jackc/pgx/v5"
	"github.com/jackc/pgx/v5/pgxpool"
	"github.com/joho/godotenv"
	"go.uber.org/zap"
)

var pool *pgxpool.Pool

type myQueryTracer struct {
	logger *zap.SugaredLogger
}

func (tracer *myQueryTracer) TraceQueryStart(
	ctx context.Context,
	_ *pgx.Conn,
	data pgx.TraceQueryStartData,
) context.Context {
	tracer.logger.Debugw("Executing command", "sql", data.SQL, "args", data.Args)

	return ctx
}

func (tracer *myQueryTracer) TraceQueryEnd(ctx context.Context, conn *pgx.Conn, data pgx.TraceQueryEndData) {
}

// not sure if every function should return err as well... i think i handle everything inside
func initDatabase() *pgxpool.Pool {
	err := godotenv.Load("../.env")
	if err != nil {
		logger.Warnf("Error loading .env file: %v\n", err)
	}

	connStr := fmt.Sprintf(
		"host=%s user=%s password=%s dbname=%s sslmode=disable",
		os.Getenv("POSTGRES_HOST"),
		os.Getenv("POSTGRES_USER"),
		os.Getenv("POSTGRES_PASSWORD"),
		os.Getenv("POSTGRES_DB"),
	)
	dbConfig, err := pgxpool.ParseConfig(connStr)
	if err != nil {
		logger.Fatalf("Unable to parse connString: %v\n", err)
	}

	dbConfig.ConnConfig.Tracer = &myQueryTracer{
		logger: logger,
	}

	pool, err := pgxpool.NewWithConfig(context.Background(), dbConfig)
	if err != nil {
		logger.Fatalf("Unable to create connection pool: %v", err)
	}

	err = pool.Ping(context.Background())
	if err != nil {
		logger.Fatalf("Unable to connect to the database: %v", err)
	}

	result, err := pool.Exec(context.Background(), `
  DROP TABLE IF EXISTS recipes;
  `)
	if err != nil {
		logger.Fatalf("Failed to DROP recipes %v", err)
	}
	logger.Info(result.String())
	result, err = pool.Exec(context.Background(), `
  CREATE TABLE recipes (
      id    UUID PRIMARY KEY,
      name  TEXT NOT NULL CONSTRAINT non_empty_name CHECK (name <> ''),
      steps TEXT NOT NULL CONSTRAINT non_empty_steps CHECK (steps <> '')
  )
  `)
	if err != nil {
		logger.Fatalf("Failed to create table recipes %v", err)
	}
	logger.Info(result.String())

	// for debugging
	sqlString := `
  INSERT INTO recipes (id, name, steps) VALUES ('01947b35-2a34-78ef-8119-1f39ff636ff8', 'cake', 'cook egg!');
  `
	result, err = pool.Exec(context.Background(), sqlString)
	if err != nil {
		logger.Errorf("Error inserting default recipe into table: %v\n", err)
	}
	logger.Info(result.String())

	return pool
}

func AddRecipe(recipe Recipe) error {
	sqlString := `
  INSERT INTO recipes (id, name, steps) VALUES ($1, $2, $3);
  `
	result, err := pool.Exec(context.Background(), sqlString, recipe.Id, recipe.Title, recipe.Steps)
	if err != nil {
		logger.Errorf("Error inserting recipe into table: %v\n", err)
	}
	logger.Info(result.String())

	return err
}

func GetAllRecipesDb() ([]Recipe, error) {
	sqlString := `
  SELECT id, name, steps FROM recipes ORDER BY id DESC
  `
	result, err := pool.Query(context.Background(), sqlString)
	if err != nil {
		logger.Errorf("Error inserting recipe into table: %v\n", err)
	}
	defer result.Close()
	var recipes []Recipe
	for result.Next() {
		var recipe Recipe
		err := result.Scan(&recipe.Id, &recipe.Title, &recipe.Steps)
		logger.Debug(recipe)
		if err != nil {
			logger.Errorf("Error getting recipe into table: %v\n", err)
			return nil, err
		}
		recipes = append(recipes, recipe)
	}
	logger.Info(recipes)
	return recipes, nil
}

func GetRecipeByIdDb(id string) (Recipe, error) {
	sqlString := `
  SELECT id, name, steps FROM recipes 
  WHERE id = ($1)
  `
	result := pool.QueryRow(context.Background(), sqlString, id)
	var recipe Recipe
	err := result.Scan(&recipe.Id, &recipe.Title, &recipe.Steps)
	if err != nil {
		logger.Errorf("Error getting recipe by id %v table: %v\n", id, err)
	}
	return recipe, err
}

func UpdateRecipe(recipe Recipe) error {
	sqlString := `
  UPDATE recipes SET name = ($1), steps = ($2)
  WHERE id = ($3)
  `

	result, err := pool.Exec(context.Background(), sqlString, recipe.Title, recipe.Steps, recipe.Id)
	if err != nil {
		logger.Errorf("Error updating recipe %v into table: %v\n", recipe.Id, err)
		return err
	}
	logger.Info(result.String())
	return nil
}

func DeleteRecipe(id string) error {
	sqlString := `
  DELETE FROM recipes
  WHERE id = ($1)
  IS TRUE RETURNING *
  `

	result, err := pool.Exec(context.Background(), sqlString, id)
	if err != nil {
		logger.Errorf("Error deleting recipe %v into table: %v\n", id, err)
	}
	logger.Info(result.String())
	return err
}
