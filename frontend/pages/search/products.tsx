import { useRouter } from "next/router";

export default function SearchUserPage() {
  const router = useRouter();
  return (
    <>
      <div className="border-b border-gray-200 bg-white px-4 py-5 sm:px-6">
        <h3 className="text-base font-semibold leading-6 text-gray-900">Pesquisas de Produtos</h3>
      </div>
      <div className="bg-white shadow sm:rounded-lg">
        <div className="px-4 py-5 sm:p-6">
          <h3 className="text-base font-semibold leading-6 text-gray-900">Pesquisa por nome</h3>
          <div className="mt-2 max-w-xl text-sm text-gray-500">
            <p>
              Pesquisar produtos por nome parecido, ordenados por preco
            </p>
          </div>
          <div className="mt-5">
            <button
              type="button"
              className="inline-flex items-center rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-500"
              onClick={() => { router.push(`/products/search`) }}
            >
              Ir para pagina
            </button>
          </div>
        </div>
      </div>

      <div className="bg-white shadow sm:rounded-lg">
        <div className="px-4 py-5 sm:p-6">
          <h3 className="text-base font-semibold leading-6 text-gray-900">Pesquisa acima de certo preco</h3>
          <div className="mt-2 max-w-xl text-sm text-gray-500">
            <p>
              Procurar produtos acima de um determinado preco
            </p>
          </div>
          <div className="mt-5">
            <button
              type="button"
              className="inline-flex items-center rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-500"
              onClick={() => { router.push(`/products/search/price`) }}
            >
              Ir para pagina
            </button>
          </div>
        </div>
      </div>
    </>
  )
}
