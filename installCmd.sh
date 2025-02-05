#/bin/bash
echo "alias hotkey='cat ~/Study/hotKey'" >> ~/.zshrc
echo "alias cargow='cargo watch -x run'" >> ~/.zshrc


## 别名
alias gitdayline='git log --author=zhouzheng --since=2.day.ago --shortstat'
alias gitdaylines='git log --shortstat --author=zhouzheng --since=midnight | grep -E "insertions|deletion"'
alias gitdayRange='git log --shortstat --author=zhouzheng --since="2023-04-25" --until="2023-04-26"| grep -E "Date|insertions|deletion"'

source ~/.zshrc
