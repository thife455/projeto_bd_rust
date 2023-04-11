import { useRouter } from "next/router";
import { useQuery } from "react-query";
import GymTable from "../../../../components/GymTable";
import { api } from "../../../../utils/api";

export default function SearchGymPage() {
  const router = useRouter();
  const slug = typeof router.query?.slug === "string" ? router.query.slug : "";

  const { data, error, isLoading } = useQuery("users", async () => {
    const users = await api.get(`/gym/${slug}/name`);
    return users.data;
  },
    { enabled: slug.length > 0 }
  );

  if (isLoading) {
    return <div>Is Loading...</div>
  }


  return (
    <GymTable data={data} />
  );
}
