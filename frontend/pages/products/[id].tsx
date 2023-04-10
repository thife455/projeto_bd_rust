import { useRouter } from "next/router";
import { useQuery } from "react-query";
import { api } from "../../utils/api";

export default function ProductInfoPage() {
  const router = useRouter();
  const id = typeof router.query?.id === "string" ? router.query.id : "";

  const { data, error, isLoading } = useQuery(
    ["products", id],
    async () => {
      const gym = await api.get(`/gym/${id}`);
      const products = await api.get(`/gym/${id}/products`);
      const data = {
        ...gym.data,
        products: products.data,
      };
      return data;
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
    <div className="overflow-hidden bg-white shadow sm:rounded-lg">
      <div className="px-4 py-5 sm:px-6">
        <h3 className="text-base font-semibold leading-6 text-gray-900">
          {data?.name}
        </h3>
        <p className="mt-1 max-w-2xl text-sm text-gray-500">
          Dados da Academia
        </p>
      </div>
      <div className="border-t border-gray-200 px-4 py-5 sm:p-0">
        <dl className="sm:divide-y sm:divide-gray-200">
          <div className="py-4 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6 sm:py-5">
            <dt className="text-sm font-medium text-gray-500">Endereco</dt>
            <dd className="mt-1 text-sm text-gray-900 sm:col-span-2 sm:mt-0">
              {data?.address}
            </dd>
          </div>
        </dl>
      </div>
      <div className="border-t border-gray-200 px-4 py-5 sm:p-0">
        <dl className="sm:divide-y sm:divide-gray-200">
          <div className="py-4 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6 sm:py-5">
            <dt className="text-sm font-medium text-gray-500">Cidade</dt>
            <dd className="mt-1 text-sm text-gray-900 sm:col-span-2 sm:mt-0">
              {data?.city}
            </dd>
          </div>
        </dl>
      </div>
      <div className="border-t border-gray-200 px-4 py-5 sm:p-0">
        <dl className="sm:divide-y sm:divide-gray-200">
          <div className="py-4 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6 sm:py-5">
          </div>
        </dl>
      </div>
      <ProductTable products={data?.products} />
    </div>
  );
}
