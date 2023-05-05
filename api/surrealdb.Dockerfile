# Use the official SurrealDB image
FROM surrealdb/surrealdb:latest

# Expose the custom port for SurrealDB
EXPOSE 7070

# Create a volume for data persistence
VOLUME /surrealdb_data

# Run SurrealDB server with the volume as the data directory and the custom port
CMD ["start","--log","trace","--user","root","--pass","root", "--bind", "0.0.0.0:7070", "file:/surrealdb_data"]
