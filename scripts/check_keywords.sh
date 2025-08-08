#!/bin/bash

# check_keywords.sh
# This script checks a log file for a list of required keywords.
# It returns an exit code of 0 if all keywords are found, and 1 otherwise.

LOG_FILE=$1
KEYWORDS_JSON=$2

# Check if the log file exists
if [ ! -f "$LOG_FILE" ]; then
    echo "❌ Log file not found: $LOG_FILE"
    exit 1
fi

# Check if jq is installed (used for parsing JSON)
if ! command -v jq &> /dev/null
then
    echo "❌ jq could not be found. Please install it."
    exit 1
fi

echo "Checking for expected keywords in $LOG_FILE..."
export missing_keywords

# Iterate over the keywords from the JSON array
while read keyword; do
    if ! grep -q "$keyword" "$LOG_FILE"; then
        echo "❌ Missing keyword: $keyword"
        missing_keywords=$((missing_keywords+1))
    else
        echo "✅ Found keyword: $keyword"
    fi
done < <(echo "$KEYWORDS_JSON" | jq -r '.[]')

# If any keywords were missing, exit with an error code
if [ "$missing_keywords" -gt 0 ]; then
    echo "❌ Missing keywords found. Full log content:"
    cat "$LOG_FILE"
    exit 1
else
    echo "✅ All expected keywords found in log!"
    exit 0
fi

