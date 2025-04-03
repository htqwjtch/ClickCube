FROM ubuntu:24.04

RUN apt update && apt install -y curl && \
    curl -fsSL https://deb.nodesource.com/setup_20.x | bash - && \
    apt install -y nodejs && \
    npm install -g npm@latest && \
    apt clean && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/front

RUN npm install react-dom@19

RUN npm install express axios @react-three/fiber @react-three/drei three --save

RUN npm install eslint --save-dev

COPY . .

CMD ["npm", "start"]


# "dbaeumer.vscode-eslint"