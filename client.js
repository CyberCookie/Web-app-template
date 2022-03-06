import { render } from 'react-dom'
import ws from '/home/cybercookie/.npm-packages/lib/node_modules/siegel/lib_client/client_core/network/socket'
// import ws from 'siegel/lib_client/client_core/network/socket'


const wsConnection = ws({
    url: '127.0.0.1',
    port: 3012
})
.on('msg', data => {
    console.log(data)
})
console.log(wsConnection)
wsConnection.send('msg', { qwerty: 123 })



render(
    'hello',
    root
)