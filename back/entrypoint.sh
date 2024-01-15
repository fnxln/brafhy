#!/bin/bash

# Wait for the database to be ready
# Include logic to check for database readiness

# Run migrations
sqlx migrate run

# Start the application
exec "$@"