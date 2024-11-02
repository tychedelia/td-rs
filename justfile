build PLUGIN:
    cargo xtask build {{PLUGIN}}

install PLUGIN:
    cargo xtask install {{PLUGIN}}

dev PLUGIN:
    bacon dev -- {{PLUGIN}}

list-plugins:
    cargo xtask list-plugins