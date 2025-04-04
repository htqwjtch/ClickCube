FROM ubuntu:24.04

RUN apt update && \
    apt install -y git-all && \
    apt install -y curl && \
    curl -fsSL https://deb.nodesource.com/setup_20.x | bash - && \
    apt install -y nodejs && \
    npm install -g npm@latest && \
    apt clean && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/front

COPY . .

RUN npm install express && npm install axios && npm install eslint

RUN npm install git+https://github.com/JithuTholoor/RubixCube.git

CMD ["npm", "start"]


# "dbaeumer.vscode-eslint"