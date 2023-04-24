#/bin/bash
echo "alias hotkey='cat ~/Study/hotKey'" >> ~/.zshrc
echo "alias cargow='cargo watch -x run'" >> ~/.zshrc
mv ~/.ssh/config ~/.ssh/config_remarks
ln -s $PWD/ssh_config  ~/.ssh/config

## 别名
alias gitdayline='git log --author=zhouzheng --since=1.day.ago --shortstat'
alias gitdaylines='git log --shortstat --author=zhouzheng --since=midnight | grep -E "insertions|deletion"'

source ~/.zshrc
