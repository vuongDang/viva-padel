# Stage 1: Build the Expo Web application
FROM node:20-alpine AS builder

WORKDIR /app

# Copy package configurations first to leverage Docker layer caching
COPY package*.json ./

# Install project dependencies
RUN npm install

# Copy the rest of your source code
COPY . .

# Build the web application (using the 'single' output configured in app.json)
# This will output files to the `dist` directory by default in Expo >= 50
RUN npx expo export -p web

# Stage 2: Serve the application with Nginx
FROM nginx:alpine

# Copy the built assets from the builder stage to Nginx's serving directory
COPY --from=builder /app/dist /usr/share/nginx/html

# Update Nginx default config to fallback to index.html for Single Page Application routing
# This is crucial so React Navigation works on page refresh instead of returning 404
RUN sed -i '/location \/ {/a \        try_files $uri $uri/ /index.html;' /etc/nginx/conf.d/default.conf

# Expose port 80 to the outside world
EXPOSE 80

# Start Nginx in the foreground
CMD ["nginx", "-g", "daemon off;"]
