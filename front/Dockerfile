FROM ubuntu:24.04

RUN apt update && \
    apt install -y curl && \
    curl -fsSL https://deb.nodesource.com/setup_20.x | bash - && \
    apt install -y nodejs && \
    npm install -g npm@latest && \
    apt clean && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src

RUN rm -rf front

RUN npx create-react-app front

WORKDIR /usr/src/front

RUN npm install express axios eslint @react-three/fiber @react-three/drei

RUN rm -rf public && mkdir public
COPY public ./public
RUN rm -rf src && mkdir src
COPY src ./src
COPY Dockerfile .

ENV PORT=3013

CMD ["npm", "start", "--", "--host", "0.0.0.0"]


# "dbaeumer.vscode-eslint"