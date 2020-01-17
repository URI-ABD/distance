FROM archlinux/base

RUN pacman -Syu --noconfirm -y

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="${HOME}/.cargo/bin:${PATH}"

RUN pacman -S --noconfirm \
    python \
    git \
    zsh \
    base-devel \
    openssh

# Oh My Zsh
RUN sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"

####################
# Install YAY
####################

# makepkg cannot (and should not) be run as root:
RUN useradd -m notroot

# Allow notroot to run stuff as root (to install dependencies):
RUN echo "notroot ALL=(ALL) NOPASSWD: ALL" > /etc/sudoers.d/notroot

# Continue execution (and CMD) as notroot:
USER notroot
WORKDIR /home/notroot

# Auto-fetch GPG keys (for checking signatures):
RUN mkdir .gnupg && \
    touch .gnupg/gpg.conf && \
    echo "keyserver-options auto-key-retrieve" > .gnupg/gpg.conf

# Install yay (for building AUR dependencies):
RUN git clone https://aur.archlinux.org/yay-bin.git && \
    cd yay-bin && \
    makepkg --noconfirm --syncdeps --rmdeps --install --clean


####################
# Anything from AUR
####################

RUN git clone https://aur.archlinux.org/git-extras.git && cd git-extras && makepkg -si --noconfirm


###################
# Wrap it up
###################

USER root
ENTRYPOINT "make init"
