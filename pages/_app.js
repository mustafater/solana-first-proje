import '@/styles/globals.css'

const WalletConnectionProvider = dynamic(() => import('../context/WalletConnectionProvider'), {
  ssr: false,
})


export default function App({ Component, pageProps }) {

  <WalletConnectionProvider>
  return <Component {...pageProps} />
  </WalletConnectionProvider>
}
