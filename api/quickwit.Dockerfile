FROM quickwit/quickwit

EXPOSE 6060

VOLUME /quickwit/qwdata

# Run SurrealDB server with the volume as the data directory and the custom port
CMD ["run"]
