
import { useRouter } from "next/router";
import { useState } from "react";

const GymSearch = () => {

  const [name, setName] = useState("");
  const router = useRouter();

  return (
    <div className="m-6">
      <div className="w-64">
        <label
          htmlFor="email"
          className="block text-sm font-medium leading-6 text-gray-900"
        >
          Nome da academia
        </label>
        <div className="mt-2">
          <input
            className="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
            placeholder="Sigla da cidade a ser perquisada"
            onChange={(e) => {
              setName(e.target.value);
            }}
          />
        </div>
      </div>
      <button
        type="button"
        className="my-6 rounded-md bg-indigo-600 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
        onClick={() => { router.push(`/gyms/search/${name}/name`) }}
      >
        Pesquisar
      </button>
    </div>)
}

export default GymSearch
