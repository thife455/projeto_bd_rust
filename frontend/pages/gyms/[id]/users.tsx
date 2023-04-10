import { useRouter } from "next/router";
import { useQuery } from "react-query";
import { api } from "../../../utils/api";
import ProductTable from "../../../components/ProductTable";
import UserTable from "../../../components/UserTable";

export default function GymInfoPage() {
  const router = useRouter();
  const id = typeof router.query?.id === "string" ? router.query.id : "";

  const { data, error, isLoading } = useQuery(
    ["gym", id],
    async () => {
      const gym = await api.get(`/gym/${id}/users`);
      console.log(data)
      return gym.data;
    },
    { enabled: id.length > 0 }
  );

  if (isLoading) {
    return <>Loading ...</>;
  }

  if (error) {
    console.log(error);
    return <>Error ...</>;
  }

  return (
    <UserTable data={data} />
  );
}
