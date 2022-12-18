import { Address, Student, Department, StudentReg } from "./types";
import { AstContext } from "./astkn";
/**
* Student Module
* 
* - Contains definitions for Student Registration and login
* - Two functions to register and login user
* - More student functionalities to fetch event data is described in event module .
* eg :- 
* @example 
*```ts 
* const conductedEvents=await conductedEvents(addr);
* const currEventState=await getCurEventState(addr);
* ```
*
* Registers the User account on SToken System
* @param addr - Wallet Address of the Student to register 
* @param _std - Instance of StudentReg required for registration 
* @returns the response of registeration , `true` if registered successfully or `false` if registration fails
*
*/
export const regStudent = async (name: string, admn_no: number, department: number, astCtx: AstContext): Promise<Boolean> => {
  try {
    await astCtx.student?.registerStudent(name, admn_no, department);
    return true;
  } catch (err) {
    return false;
  }
}

/**
* Display the Student Profile Detials on SToken System
* @param addr -  Wallet Address of the Student to Display Student Profile 
* @returns `Student` Object corresponding to the address    
*/
export const dispStudent = async (ctx: AstContext): Promise<Student> => {
  return await ctx?.student?.getStudent();
}


