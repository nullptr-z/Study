#/bin/bash
echo "alias hotkey='cat ~/Study/hotKey'" >> ~/.zshrc
echo "alias cargow='cargo watch -x run'" >> ~/.zshrc
mv ~/.ssh/config ~/.ssh/config_remarks
ln -s $PWD/ssh_config  ~/.ssh/config
	source ~/.zshrc
