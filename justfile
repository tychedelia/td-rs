build PLUGIN:
    cargo xtask build {{PLUGIN}}

install PLUGIN:
    cargo xtask install {{PLUGIN}}

list-plugins:
    cargo xtask list-plugins