ARG VARIANT="3.10-bullseye"
FROM mcr.microsoft.com/vscode/devcontainers/python:0-${VARIANT}

ARG NODE_VERSION="none"
RUN if [ "${NODE_VERSION}" != "none" ]; then su vscode -c "umask 0002 && . /usr/local/share/nvm/nvm.sh && nvm install ${NODE_VERSION} 2>&1"; fi

# [Optional] Uncomment this section to install additional packages.
# RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
#     && apt-get -y install --no-install-recommends <your-package-list-here>

WORKDIR /home/

COPY . .

RUN bash ./setup.sh

ENV PATH="/root/.cargo/bin:$PATH"

# install python stuff
RUN pip install mypy wasmtime

# install wasi-sdk
RUN cd /opt && curl -L https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-16/wasi-sdk-16.0-linux.tar.gz \
    | tar -xz
ENV WASI_SDK_PATH=/opt/wasi-sdk-16.0

RUN echo 'alias clang=/opt/wasi-sdk-16.0/bin/clang' >> /etc/bash.bashrc
RUN echo 'alias clang++=/opt/wasi-sdk-16.0/bin/clang++' >> /etc/bash.bashrc

# Go installation, see https://go.dev/doc/install
ARG GO_URL="https://go.dev/dl/go1.17.9.linux-amd64.tar.gz"
RUN curl -sL "$GO_URL" | tar -xzf - -C /usr/local
ENV PATH "$PATH:/usr/local/go/bin"

# TinyGo installation, see https://tinygo.org/getting-started/install/linux/ for instructions
ARG TINYGO_URL="https://github.com/tinygo-org/tinygo/releases/download/v0.22.0/tinygo_0.22.0_amd64.deb"
RUN curl -sL "$TINYGO_URL" -o tinygo_amd64.deb && dpkg -i tinygo_amd64.deb && rm tinygo_amd64.deb

# Install the gopls Go Language Server, see https://github.com/golang/tools/tree/master/gopls
RUN go install golang.org/x/tools/gopls@latest

# Install Grain, see https://grain-lang.org/docs/getting_grain
ARG GRAIN_URL="https://github.com/grain-lang/grain/releases/download/grain-v0.4.7/grain-linux-x64"
RUN curl -sL "$GRAIN_URL" -o /usr/local/bin/grain && chmod +x /usr/local/bin/grain

# Install AssemblyScript (Node.js dependency), see https://www.assemblyscript.org/getting-started.html
ARG NODEJS_URL="https://nodejs.org/dist/v17.9.0/node-v17.9.0-linux-x64.tar.xz"
RUN curl -sL "$NODEJS_URL" | tar -xJf - -C /usr/local --strip-components=1 --no-same-owner
RUN npm install -g assemblyscript

# Install other js dep
RUN npm install