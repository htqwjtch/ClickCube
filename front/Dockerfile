FROM ubuntu:24.04

RUN apt update && apt install -y curl && \
    curl -fsSL https://deb.nodesource.com/setup_20.x | bash - && \
    apt install -y nodejs && \
    npm install -g npm@latest && \
    apt clean && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/front

RUN npm init -y && npm install express && npm install -g eslint && npm install axios

COPY . .

CMD ["npm", "start"]


# "dbaeumer.vscode-eslint"