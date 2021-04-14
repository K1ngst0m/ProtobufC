const {createProxyMiddleware}=require('http-proxy-middleware')

module.exports=function (app) {
    app.use(
        createProxyMiddleware('/api',{
            target:'http://192.168.1.141:8000/protoc',
            changeOrigin:true,
            pathRewrite:{
                '^/api':''
            },
            "secure":true
        })
    )
}
