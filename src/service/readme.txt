为什么需要service层？
1.封装跨表业务逻辑
2.redis,mq,es,grpc等组件逻辑
3.调用三方服务，如微信，短信，支付宝,OSS
4.应用服务功能
5.handler层负责mvc中的controller，只负责记录日志，调用service层逻辑