FROM mcr.microsoft.com/devcontainers/base:bullseye

ARG TASK_VERSION=3.40.0
ARG ZIG_VERSION=0.13.0

# install task and enable Taskfile completion
RUN wget https://github.com/go-task/task/raw/99ed3001f0813149c285714574ce24686a30478f/install-task.sh \
        -O - | sh -s -- -d -b /usr/local/bin v${TASK_VERSION} && \
    wget https://github.com/go-task/task/raw/a1b3bb03ed7f7339ba7a03db0bbf8e5ca35eef95/completion/bash/task.bash \
        -O /etc/bash_completion.d/task && \
    chmod 755 /etc/bash_completion.d/task

# install zig for cargo-zigbuild
RUN wget https://ziglang.org/download/${ZIG_VERSION}/zig-linux-x86_64-${ZIG_VERSION}.tar.xz -O - | tar -xJf - -C /usr/local
ENV PATH=/usr/local/zig-linux-x86_64-${ZIG_VERSION}:$PATH

# install GitHub CLI
RUN sudo mkdir -p -m 755 /etc/apt/keyrings \
    && wget -qO- https://cli.github.com/packages/githubcli-archive-keyring.gpg | sudo tee /etc/apt/keyrings/githubcli-archive-keyring.gpg > /dev/null \
    && sudo chmod go+r /etc/apt/keyrings/githubcli-archive-keyring.gpg \
    && echo "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main" | sudo tee /etc/apt/sources.list.d/github-cli.list > /dev/null \
    && sudo apt update \
    && sudo apt install gh -y \
    && rm -rf /var/lib/apt/lists/*
