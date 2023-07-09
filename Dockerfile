FROM multiarch/cross

# Install Python
RUN apt-get update && apt-get install -y python3

# Set the entrypoint or command to run your application
CMD [ "cargo", "--version" ]