#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>

#define MAX_LEVELS 100 // Maximum levels per report

// Function to check if a report is safe
bool check_safety(const int levels[], int size) {
    bool increasing = true;
    bool decreasing = true;

    // Check the differences between adjacent levels
    for (int i = 0; i < size - 1; i++) {
        int diff = levels[i + 1] - levels[i];

        // Check if the difference is within the allowed range (1 to 3)
        if (diff == 0 || abs(diff) > 3) {
            return false; // Invalid difference
        }

        // Check if the report is strictly increasing or decreasing
        if (diff > 0) {
            decreasing = false; // Not decreasing
        } else if (diff < 0) {
            increasing = false; // Not increasing
        }
    }

    return (increasing || decreasing); // Safe if strictly increasing or decreasing
}

// Function to check if removing one level makes the report safe
bool check_safety_with_one_removal(int levels[], int size) {
    // Try removing each level once and check if the resulting report is safe
    for (int i = 0; i < size; i++) {
        // Create a temporary array with one less element
        int temp_levels[MAX_LEVELS];
        int temp_size = 0;

        // Copy all levels except the one at index 'i'
        for (int j = 0; j < size; j++) {
            if (j != i) {
                temp_levels[temp_size++] = levels[j];
            }
        }

        // Check if the report is safe after removal
        if (check_safety(temp_levels, temp_size)) {
            return true;
        }
    }

    return false; // If no removal makes it safe, return false
}

// Function to read lines from the input file and process them
int main() {
    FILE *file = fopen("input.txt", "r");
    if (file == NULL) {
        perror("Failed to open file");
        return 1;
    }

    int levels[MAX_LEVELS];
    int safe_count = 0;
    char line[1024];  // Buffer to read each line

    // Read each line from the file
    while (fgets(line, sizeof(line), file) != NULL) {
        int level_count = 0;

        // Tokenize the line to get the levels
        char *token = strtok(line, " \t\n");
        while (token != NULL) {
            levels[level_count++] = atoi(token); // Convert token to integer
            token = strtok(NULL, " \t\n");
        }

        // Check if the current report is safe without removal or with one level removal
        if (check_safety(levels, level_count) || check_safety_with_one_removal(levels, level_count)) {
            safe_count++;
        }
    }

    printf("Total Safe Reports: %d\n", safe_count);

    fclose(file);
    return 0;
}

