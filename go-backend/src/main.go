package main

import (
	"errors"
	"net/http"

	"github.com/google/uuid"
	"github.com/gorilla/mux"

	"go.uber.org/zap"
)

type PostRecipeInput struct {
	Title string `json:"title"`
	Steps string `json:"steps"`
}

type Recipe struct {
	Id    uuid.UUID `json:"id"`
	Title string    `json:"title"`
	Steps string    `json:"steps"`
}

var logger *zap.SugaredLogger

func main() {
	zapLogger, err := zap.NewDevelopment() // Use zap.NewProduction() for production
	if err != nil {
		panic("Failed to initialize Zap logger: " + err.Error())
	}
	defer zapLogger.Sync()
	logger = zapLogger.Sugar()

	pool = initDatabase()
	defer pool.Close()

	router := mux.NewRouter()
	router.Use(LoggingMiddleware(logger))
	router.HandleFunc("/recipes", PostRecipe).Methods("POST")
	router.HandleFunc("/recipes", GetAllRecipes).Methods("GET")
	router.HandleFunc("/recipes/{id}", GetRecipeById).Methods("GET")
	router.HandleFunc("/recipes/{id}", DeleteRecipeById).Methods("DELETE")
	router.HandleFunc("/recipes/{id}", UpdateRecipeById).Methods("POST")
	err = http.ListenAndServe(":3333", router)
	if errors.Is(err, http.ErrServerClosed) {
		logger.Info("server closed\n")
	} else if err != nil {
		logger.Fatalf("error starting server: %s\n", err)
	}
}
