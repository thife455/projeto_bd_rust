import { QueryClient } from "react-query";
import "../styles/globals.css";

const queryClient = new QueryClient();

export default function MyApp({ Component, pageProps }) {
  return <Component {...pageProps} />;
}
