import { useRouter } from "next/router";
import { useQuery } from "react-query";
import { api } from "../../../utils/api";
import UserTable from "../../../components/UserTable";

export default function SearchUserPage() {
  const router = useRouter();
  const nome = typeof router.query?.nome === "string" ? router.query.nome : "";
  const { data, error, isLoading } = useQuery("users", async () => {
    const users = await api.get(`/user/name/${nome}`);
    return users.data;
  },
    { enabled: nome.length > 0 }
  );

  if (isLoading) {
    return <div>Is Loading...</div>
  }


  return (
    <UserTable data={data} />
  );
}
