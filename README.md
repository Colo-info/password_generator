# password_generator
我是Rust小白，请各位大佬手下留情。\t
在实际项目中需要使用强安全的密码，为了满足在离线环境下的密码自动生成。我将通过Rust编写相关程序，并生成相关软件。\t
## 一、软件结构：
1）复杂字符串生成\t
	复杂密码的生成规则主要是2条：\t
		1）密码长度，最低8位密码；\t
		2）密码包含的类型，大小写字母、特殊符号、数字；\t
	密码长度：\t
		1）根据用户要求设置密码长度；\t
		2）客户可自行输入密码长度；\t
	密码包含：\t
		1）默认设置；\t
2）通过UI的方式显示结果，并可复制;\t
## 二、版本展示
### v1.0(初版)
已实现：\t
1）根据客户密码长度要求自行生成复杂密码，对于生成的复杂密码保存至passwd.txt文件下；\t
存在缺失：\t
1）密码长度最长为255个字符，后期将进行扩展；\t
2）不能对于密码包含内容进行限制；\t
3）没有通过UI界面进行展示，需要自行打开passwd.txt文件；\t
\t
版本网址:https://github.com/Colo-info/password_generator/tree/v1.0
