import { QueryClient, QueryClientProvider } from "react-query";
import "../styles/globals.css";
import Navbar from "../components/Navbar";

const queryClient = new QueryClient();

export default function MyApp({ Component, pageProps }) {
  return (
    <QueryClientProvider client={queryClient}>
      <Navbar></Navbar>
      <Component {...pageProps} />
    </QueryClientProvider>
  );
}
