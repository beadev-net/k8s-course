FROM node:lts-alpine3.19

# Create app directory 
RUN mkdir -p /usr/src/app

# Set the working directory
WORKDIR /usr/src/app

# Copy the current directory contents into the container at /usr/src/app
COPY . /usr/src/app

# Install app dependencies
RUN npm install

# Run start command on package.json
CMD ["npm", "start"]