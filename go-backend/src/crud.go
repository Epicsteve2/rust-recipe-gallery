package main

import (
	"encoding/json"
	"fmt"
	"net/http"

	"github.com/google/uuid"
	"github.com/gorilla/mux"
	"go.uber.org/zap"
)

// LoggingMiddleware is a middleware that logs HTTP requests
func LoggingMiddleware(logger *zap.SugaredLogger) func(http.Handler) http.Handler {
	return func(next http.Handler) http.Handler {
		return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
			// start := time.Now()
			logger.Infow("HTTP Request",
				// "duration", time.Since(start), // don't carre for now
				"method", r.Method,
				"url", r.URL.String(),
				"host", r.Host,
				"remote_addr", r.RemoteAddr,
				// "body", r.Body, // not supposed to read multiple times. like rust!!
			)
			next.ServeHTTP(w, r)
		})
	}
}

func PostRecipe(w http.ResponseWriter, req *http.Request) {
	var recipeInput PostRecipeInput
	err := json.NewDecoder(req.Body).Decode(&recipeInput)
	if err != nil {
		http.Error(w, "Invalid input", http.StatusBadRequest)
	}
	logger.Infow("recipe input", "recipe", recipeInput)

	id, err := uuid.NewV7()
	if err != nil {
		logger.Errorf("Error generating uuid v7: %v", err)
		http.Error(w, fmt.Sprintf("Error generating uuid v7: %v", err), http.StatusInternalServerError)
		return
	}

	recipe := Recipe{
		id,
		recipeInput.Title,
		recipeInput.Steps,
	}
	err = AddRecipe(recipe)
	if err != nil {
		http.Error(w, fmt.Sprintf("Error input to db: %v", err), http.StatusInternalServerError)
		return
	}
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusCreated)
	json.NewEncoder(w).Encode(recipe)
}

func GetAllRecipes(w http.ResponseWriter, req *http.Request) {
	recipes, err := GetAllRecipesDb()
	if err != nil {
		http.Error(w, "Error getting recipes", http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(recipes)
}

func GetRecipeById(w http.ResponseWriter, req *http.Request) {
	vars := mux.Vars(req)

	id := vars["id"]
	recipe, err := GetRecipeByIdDb(id)
	if err != nil {
		http.Error(w, fmt.Sprintf("Error getting recipe: %v %v", id, err), http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(recipe)
}

func DeleteRecipeById(w http.ResponseWriter, req *http.Request) {
	vars := mux.Vars(req)

	id := vars["id"]
	err := DeleteRecipe(id)
	if err != nil {
		http.Error(w, fmt.Sprintf("Error deleting recipe: %v %v", id, err), http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusNoContent)
}

func UpdateRecipeById(w http.ResponseWriter, req *http.Request) {
	var recipeInput PostRecipeInput
	err := json.NewDecoder(req.Body).Decode(&recipeInput)
	if err != nil {
		http.Error(w, fmt.Sprintf("Invalid input %v", err), http.StatusBadRequest)
	}
	logger.Infow("recipe input", "recipe", recipeInput)

	vars := mux.Vars(req)

	id := vars["id"]
	uuidId, err := uuid.Parse(id)
	if err != nil {
		http.Error(w, fmt.Sprintf("Invalid uuid %v %v", id, err), http.StatusBadRequest)
	}

	recipe := Recipe{
		Id:    uuidId,
		Title: recipeInput.Title,
		Steps: recipeInput.Steps,
	}

	err = UpdateRecipe(recipe)
	if err != nil {
		http.Error(w, fmt.Sprintf("Error updating recipe %v %v", uuidId, err), http.StatusBadRequest)
	}
}
