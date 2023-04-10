
import { useQuery } from "react-query";
import { api } from "../../utils/api";
import { useRouter } from "next/router";

type Gym = {
  name: string;
}

type Product = {
  id: string;
  name: string;
  price: number;
  Gym?: Gym
}

export default function MostSoldProductsPage() {
  const router = useRouter();
  const { data, error, isLoading } = useQuery("users", async () => {
    const products: Array<Product> = (await api.get("/products/most_sold")).data;


    for (const product of products) {
      const gym = await api.get(`/products/${product.id}/gym`)
      product.Gym = gym.data
    }


    return products;
  });

  if (isLoading) {
    return <div>Loading...</div>;
  }

  return (
    <>
      <div className="flex flex-row w-full items-center self-center justify-center">
        <h1 className="m-5 font-extrabold text-5xl">Produtos mais vendidos!</h1>
      </div>
      <table className="min-w-full divide-y divide-gray-200">
        <thead>
          <tr>
            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Nome
            </th>
            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Preco
            </th>
            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Academia
            </th>
          </tr>
        </thead>
        <tbody>
          {data?.map((product) => (
            <tr key={product.id}>
              <td className="px-6 py-4 whitespace-nowrap">{product.name}</td>
              <td className="px-6 py-4 whitespace-nowrap">{product.price}</td>
              <td className="px-6 py-4 whitespace-nowrap">{product.Gym.name}</td>
              <td className="px-6 py-4 whitespace-nowrap">
                <button onClick={() => { router.push(`products/${product.id}`) }} className="bg-blue-500 text-white rounded-md">
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
