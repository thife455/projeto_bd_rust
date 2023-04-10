import { useRouter } from "next/router";
import React from "react";
import users from "../api/users";


const UsersTable = ({ userData }) => {
   const router = useRouter();
   return (
      <>
         <form action="">
            <input type="text" placeholder="Pesquisar nome de usuário..." />
            <button onClick={() => {
               router.push(`users/${users.name}`);
            }}/>
         </form>
         
      <table className="min-w-full divide-y divide-gray-200">
         <thead>
            <tr>
            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
               ID
            </th>
            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
               Nome
            </th>
            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
               Email
            </th>
            </tr>
         </thead>
         <tbody>
            {userData.map((user) => (
            <tr key={user.id}>
               <td className="px-6 py-4 whitespace-nowrap">{user.id}</td>
               <td className="px-6 py-4 whitespace-nowrap">{user.name}</td>
               <td className="px-6 py-4 whitespace-nowrap">{user.email}
               </td>
            </tr>
            ))}
         </tbody>
      </table>
      </>
   );
};

export default UsersTable;
