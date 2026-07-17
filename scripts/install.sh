cargo install --path editor
cargo install --path seq_editor

mkdir -p $HOME/.local/bin

ln -sf $HOME/.crate/bin/gart_editor $HOME/.local/bin/gart_editor
ln -sf $HOME/.crate/bin/gart_seq_editor $HOME/.local/bin/gart_seq_editor
ln -sf $PWD/scripts/gart.sh $HOME/.local/bin/gart
