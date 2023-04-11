import { useRouter } from "next/router";
import { useState } from "react";

const ProductSearch = () => {

  const [price, setPrice] = useState(0)
  const router = useRouter();

  return (
    <div className="m-6">
      <div className="w-64">
        <label
          htmlFor="email"
          className="block text-sm font-medium leading-6 text-gray-900"
        >
          Preco do produto
        </label>
        <div className="mt-2">
          <input
            type="number"
            className="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
            placeholder="Preco minimo do produto"
            onChange={(e) => {
              setPrice(parseInt(e.target.value));
            }}
          />
        </div>
      </div>
      <button
        type="button"
        className="my-6 rounded-md bg-indigo-600 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
        onClick={() => { router.push(`/products/search/${price}/price`) }}
      >
        Pesquisar
      </button>
    </div>)
}

export default ProductSearch
