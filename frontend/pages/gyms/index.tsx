import { useQuery } from "react-query";
import { api } from "../../utils/api";
import { useRouter } from "next/router";

export default function GymPage() {
  const router = useRouter();
  const { data, error, isLoading } = useQuery("users", async () => {
    const gyms = await api.get("/gym");
    return gyms.data;
  });

  if (isLoading) {
    return <div>Loading...</div>;
  }

  return (
    <>
      <table className="min-w-full divide-y divide-gray-200">
        <thead>
          <tr>
            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Nome
            </th>
            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Endereco
            </th>
            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Cidade
            </th>
          </tr>
        </thead>
        <tbody>
          {data.map((gym) => (
            <tr key={gym.id}>
              <td className="px-6 py-4 whitespace-nowrap">{gym.name}</td>
              <td className="px-6 py-4 whitespace-nowrap">{gym.address}</td>
              <td className="px-6 py-4 whitespace-nowrap">{gym.city}</td>
              <td className="px-6 py-4 whitespace-nowrap">
                <button onClick={() => {router.push(`gyms/${gym.id}`)}} className="bg-blue-500 text-white rounded-md">
                  <p className="m-3">Detalhes</p>
                </button>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </>
  );
}
