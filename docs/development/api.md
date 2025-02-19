

### api/vikacg/v1/getIndex

path: `api/vikacg/v1/getIndex`

method: POST

desciption: Get the title content

request body/example:

response: 

```json
[
  {
    "post_type": "post",
    "id": "67498",
    "thumb": "https://xx.xx.xxx/2021/02/portfolio-4.jpg",
    "url": "https://www.xxx.xxx/p/67498.html"
  },
  {
    "post_type": "post",
    "id": "147650",
    "thumb": "https://xx.xx.xxx/2019/06/255033313ef53686886_1_avatar.jpg",
    "url": "https://www.xxx.xxx/p/147650.html"
  }
]
```

### api/b2/v1/getLatestAnnouncement

path: `api/b2/v1/getLatestAnnouncement`

method: POST

desciption: Get the latest announcement

request body/example:

```json
{
  "count": 5
}
```

response: 
```json
[
  {
    "title": "春节放假通知（蛇年快乐）",
    "href": "https://www.xx.xxx/a/290118.html",
    "thumb": "https://xx.xx.xxx/2021/06/video-model.jpg",
    "date": "2025-1-23 21:14:41",
    "desc": "尊敬的各位咔友： 我们春节要放假了，审核速度依旧但是人工复审会放缓，技术部门直接跑路到马六甲吃席，如果不是新加坡被美中两国同时断交了我们一般不会卵你，祝大家蛇年蛇满天。 From:爱你们的XX",
    "timestamp": "1737638081",
    "days": "1",
    "show": false
  },
  {
    "title": "XX道歉声明",
    "href": "https://xx.xx.xxx/a/261918.html",
    "thumb": "https://xx.xx.xxx/2024/12/photo_2024-12-13_13-29-05.jpg",
    "date": "2024-12-13 14:02:01",
    "desc": "亲爱的原神玩家及所有关注TGA的游戏爱好者： 在此，XX团队郑重向原神玩家以及所有被我们之前不当言论冒犯的朋友们道歉！ 此前，我们因对TGA（The Game Awards）榜单的某些评选结果感到愤慨，曾在情绪激动之下发表了“野鸡榜”等言辞，间接冒犯了原神玩家群体。在这里，我们为当时的情绪化表达深表歉意。然而，经历了2024年TGA榜单揭晓之后，我们不得不直言：今年的TGA榜单确实抽象得坐实了‘野&hellip;",
    "timestamp": "1734069721",
    "days": "1",
    "show": false
  },
  {
    "title": "XX极速下载页正式启用及社区投稿规则",
    "href": "https://xx.xx.xxx/a/256713.html",
    "thumb": "https://xx.xx.xxx/2021/06/video-model.jpg",
    "date": "2024-11-27 15:53:26",
    "desc": "亲爱的咔友们： 随着先前合作协议的到期，联合极速下载页现已正式更名为XX极速下载页。我们将继续为大家提供优质的下载服务，感谢大家一直以来的支持与厚爱！ 为了让更多优秀的内容被推荐给社区，我们决定开放社区投稿功能，并制定以下投稿规则： 支持移动端运行：投稿作品必须能够在移动端（最低限度支持平台为Android）运行。借助模拟器在对应移动平台运行的作品，也符合要求。 作品质量保证：提交的作品需足够优质&hellip;",
    "timestamp": "1732694006",
    "days": "3",
    "show": false
  },
  {
    "title": "XX的代码库出现泄露事故",
    "href": "https://xx.xx.xxx/a/255952.html",
    "thumb": "https://xx.xx.xxx/2021/06/video-model.jpg",
    "date": "2024-11-24 19:42:52",
    "desc": "XX的代码库出现泄露事故，我们正在努力排查可能的情况，稍后我们会进行情况通报",
    "timestamp": "1732448572",
    "days": "1",
    "show": false
  },
  {
    "title": "防范不明“付费社区”的互动，请诸位提高反诈意识",
    "href": "https://xx.xx.xxx/a/218672.html",
    "thumb": "https://xx.xx.xxx/2019/06/White-And-Blue-Elegant-Abstract-Designer-Business-Card_1_post_NmBgw9503.png",
    "date": "2024-7-23 18:36:03",
    "desc": "尊敬的咔友们： 你们好！在社区内互动时请注意防范电信诈骗，目前已有咔友报告称有所谓”付费社区“的用户主动私聊推销，并尝试引流至QQ和WeChat公众号等平台，要求被私信者支付100CNY（等值20SGD）以上才可以获取所谓的精品资源，但该咔友付费后发现下载下来的是绝区零等游戏的文件。目前咔友已经向目标引流平台投诉，并且向我们检举该用户的行为。 在此我们特别提醒，不要相信来路不明的所谓”付费社区“，&hellip;",
    "timestamp": "1721730963",
    "days": "3",
    "show": false
  }
]
```