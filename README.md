# Brush

## 简介

一款命令行环境下的刷题软件，主要功能：

* 题库题目分章节反复练习
* 错题收集和复习（错题连续做对3次，才会从错题本中清除）
* 自带题库：军事理论、近代史纲要、马原、思修（不保证时效性和正确性，请在学习前自行检查题库是否过时、正确）
* 支持导入符合规范的json格式题库

![21.12.28](21.12.28.gif)

---

## 使用步骤

### 开发环境

1. 确保本地环境安装了python3，并设置了对应的环境变量。
2. 终端下切换工作目录到此项目
3. 使用下面命令

```bash
python3 Brush.py
```

#### 再封装提示

  如你修改了项目代码，需要对项目进行再封装。请安装pyinstaller，在项目根目录下执行

```bash
pyinstaller --onefile --add-data "data;data"  brush.py --distpath .
```

### 开箱即用

  Windows环境下，直接运行brush.exe。（无需安装Python）

### 注意

   由于**每次导入题库会将./data/bank目录下的所有json题库文件一起导入，已导入的题库进度/错题数据会被重置**。

   所以，建议在当期学习开始（首次启动程序）时，就准备好要使用的题库文件，并在主界面导入题库。避免中途导入新题库，使既往学习进度丢失。

   题库位于./data/bank

---

## 建议的题库组织格式（JSON文件）

下面是一个包含两章题目的题库格式模板：

```json
{
    "name": "题库名称",
    "sections": [
        {
            "name": "第1章",
            "progress": 0,
            "problems": [
                {
                    "title": "题目1",
                    "key": "答案1",
                    "type": "题目类型1",
                    "options": [
                        "选项1",
                        "选项2",
                        "选项3",
                        "选项4"
                    ]
                },
                {
                    "title": "题目2",
                    "key": "答案2",
                    "type": "题目类型2",
                    "options": [
                        "选项1",
                        "选项2",
                        "选项3",
                        "选项4"
                    ]
                }
                // 可以添加更多题目
            ]
        },
        {
            "name": "第2章",
            "progress": 0,
            "problems": [
                {
                    "title": "题目1",
                    "key": "答案1",
                    "type": "题目类型1",
                    "options": [
                        "选项1",
                        "选项2",
                        "选项3",
                        "选项4"
                    ]
                },
                {
                    "title": "题目2",
                    "key": "答案2",
                    "type": "题目类型2",
                    "options": [
                        "选项1",
                        "选项2",
                        "选项3",
                        "选项4"
                    ]
                }
                // 可以添加更多题目
            ]
        }
    ]
}
```

#### 说明

- `"name"`：整体题库的名称

- `"sections"`：包含多个章节的数组，每个章节有自己的题目。

- 每个章节有：

  - `"name"`：章节名称，例如 "第1章"。
  - `"progress"`：进度，例如 `0`。
  - `"problems"`：包含题目的数组，每个题目有：
    - `"title"`：题目内容。
    - `"key"`：正确答案（如A、B、CD；选择题中一般正确为A、错误为B）。
    - `"type"`：题目类型（例如 "单选题", "多选题", "判断题"）。
    - `"options"`：答案选项（包含选项主干）的数组。

  未尽之处，参考自带题库。

---

## 更新日志

### 2024/05/30

- 对工程进行了封装，提供开箱即用体验

- 令刷题进度从1开始计数，更符合UI逻辑

- 更新了README

### 2022/06/21

- json的格式进行了大改，性能优化，布局更加合理

- 增加了思修的题库，个别题目有错，以后更改

- 新增马原题库

### 2022/01/06

* 去除了近代史题库中重复的题，现在总计1408道

* 将错题的连续做对的次数改为3次

### 2022/01/05

* 近代史题库更新 8，9，10章，至此完结

### 2022/01/04

* 做题时把 1 2 3 4 5 6 映射到了 A B C D E F

### 2022/01/03

* 军事理论判断题题库的修复

### 2021/12/31

* 增加错题顺序的打乱，现在做错题时是随机顺序了
* 错题改为做对4次才会从错题本中删除

### 2021/12/30

* 答题界面显示进度
* 修章节列表输入非正常范围数字退出程序
* 多选题现在可以不按照顺序输入答案了

### 2021/12/28

* 增加主菜单页面对科目做题进度的统计
* 完善了题目做完以后，任然停留在最后一道的BUG(包括列表的细节)
* 新增历史题库第7章

### 2021/12/28

* 重写了程序
* 增加了对军事理论的支持
* 支持json导入题库
* 完善了异常的处理

### 2021/12/25

* 修复选择题判断错误的bug
* 增加对Linux/Unix的支持

### 2021/12/24

* 刷题进度记录
* 错题收集功能
* 菜单列表下显示进度
* 清空错题记录和刷题进度记录
