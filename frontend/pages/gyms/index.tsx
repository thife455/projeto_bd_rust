import { useQuery } from "react-query";
import { api } from "../../utils/api";
import { useRouter } from "next/router";

export default function GymPage() {
  const router = useRouter();
  const { data, error, isLoading } = useQuery("gyms", async () => {
    const gyms = await api.get("/gym");
    return gyms.data;
  });

  if (isLoading) {
    return <div>Loading...</div>;
  }

  if (error) {
    return <div>Erro ao carregar</div>;
  }

  return (
    <>

    </>
  );
}
