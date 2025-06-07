// /Users/wenyan/mutsea/mutsea/scripts/setup_database.sh
#!/bin/bash

echo "🚀 Setting up Mutsea database for development"
echo "============================================="

# Check if PostgreSQL is running
if ! pg_isready -q; then
    echo "❌ PostgreSQL is not running!"
    echo "💡 Start PostgreSQL first:"
    echo "   - macOS: brew services start postgresql"
    echo "   - Ubuntu: sudo systemctl start postgresql"
    echo "   - Docker: docker run -d --name postgres -e POSTGRES_PASSWORD=password -p 5432:5432 postgres:13"
    exit 1
fi

echo "✅ PostgreSQL is running"

# Create database if it doesn't exist
DB_NAME="mutsea_dev"
if psql -lqt | cut -d \| -f 1 | grep -qw $DB_NAME; then
    echo "✅ Database '$DB_NAME' already exists"
else
    echo "🏗️  Creating database '$DB_NAME'..."
    createdb $DB_NAME
    echo "✅ Database '$DB_NAME' created"
fi

# Set environment variable
export DATABASE_URL="postgresql://$(whoami)@localhost:5432/$DB_NAME"
echo "🔗 DATABASE_URL: $DATABASE_URL"

# Run the setup example
echo "🏗️  Running database setup..."
cd "$(dirname "$0")/.."
cargo run --example quick_setup --features opensim-compat

echo "🎉 Database setup complete!"
echo ""
echo "💡 To use the database in your code:"
echo "   export DATABASE_URL=\"$DATABASE_URL\""
echo "   cargo run --example opensim_integration --features opensim-compat"