import { useQuery } from "react-query";
import { api } from "../../utils/api";
import { useRouter } from "next/router";

export default function UserPage() {
  const router = useRouter();
  const { data, error, isLoading } = useQuery("users", async () => {
    const users = await api.get("/user");
    return users.data;
  });

  if (isLoading) {
    return <div>Loading...</div>;
  }

  if (error) {
    return <div>Erro1!</div>;
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
              Email
            </th>
          </tr>
        </thead>
        <tbody>
          {data.map((user) => (
            <tr key={user.id}>
              <td className="px-6 py-4 whitespace-nowrap">{user.name}</td>
              <td className="px-6 py-4 whitespace-nowrap">{user.email}</td>
              <td className="px-6 py-4 whitespace-nowrap">
                <button onClick={() => {router.push(`users/${user.id}`)}} className="bg-blue-500 text-white rounded-md">
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
