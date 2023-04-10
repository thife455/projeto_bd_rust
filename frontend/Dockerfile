# Use an official Node.js runtime as a parent image
FROM node:16-alpine AS build

# Set the working directory to /app
WORKDIR /app

# Copy the package.json and yarn.lock files into the container
COPY package*.json yarn.lock ./

# Install dependencies
RUN yarn install --production --frozen-lockfile

# Copy the rest of the application files into the container
COPY . .

# Expose port 3000 for the Next.js web server
EXPOSE 3000

# Set the command to start the Next.js web server
CMD ["yarn", "dev"]
