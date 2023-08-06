FROM rust:1.71.1-slim-bullseye as base

FROM base as development

ENV HOME /root

WORKDIR ${HOME}

RUN set -ex && \
    apt-get update && \
    apt-get install -y --no-install-recommends \
            vim \
            jq \
            curl \
            git \
            && \
    apt-get upgrade -y && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# RUN git clone https://github.com/rui314/mold.git && \
#     mkdir mold/build
# 
# WORKDIR ${HOME}/mold/build
# 
# RUN git checkout v2.0.0 && \
#     ../install-build-deps.sh && \
#     cmake -DCMAKE_BUILD_TYPE=Release -DCMAKE_CXX_COMPILER=c++ .. && \
#     cmake --build . -j $(nproc) && \
#     cmake --install .

RUN curl -sSL https://github.com/rui314/mold/releases/download/v2.0.0/mold-2.0.0-x86_64-linux.tar.gz -o mold-2.0.0-x86_64-linux.tar.gz && \
    tar zxvf mold-2.0.0-x86_64-linux.tar.gz && \
    mv mold-2.0.0-x86_64-linux mold

ENV PATH ${PATH}:${HOME}/mold/bin

RUN set -ex && \
    curl -sSL https://raw.githubusercontent.com/peristrophe/dotfiles/master/.vimrc > ${HOME}/.vimrc && \
    git clone https://github.com/VundleVim/Vundle.vim.git ${HOME}/.vim/bundle/Vundle.vim && \
    vim +PluginInstall +qall

RUN echo "PS1='\[\e[1;33m\]\u@\[\e[m\]\[\e[1;32m\]\h:\[\e[m\]\[\e[1;36m\]\w$ \[\e[m\]'" >> /root/.bashrc && \
    echo "alias la='ls -lA --color=auto'" >> /root/.bashrc

