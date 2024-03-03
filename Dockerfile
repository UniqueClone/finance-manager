# Sets the base image
FROM rust:1.67

# Sets the working directory
WORKDIR /src

# Copies the current directory contents into the container at /src
COPY . .

# Installs the application
RUN cargo install --path .

# Runs the application
# The array contains the command to run the application.
# The elements of the array will be combined into a single command with spaces between the elements.
CMD [ "finance-manager" ]
