import { useRouter } from "next/router"
import { useSelector, useDispatch } from "react-redux"
import { studentSignin, adminSignin } from '../Slices/loginSlice'
import { AstContextBuilder } from "../std_api/src/astkn"
import { stdLogin, isAdmin } from "../std_api/src/events"

export default function Home() {
  const dispatch = useDispatch()
  const router = useRouter()
  async function studentLogin() {
    if (typeof window != "undefined" && typeof window.ethereum != "undefined") {
      try {
        // Metamask avalable
        // const accounts = await window.ethereum.request({ method: "eth_requestAccounts" })
        const ctx = (await (await (await new AstContextBuilder().addProvider()).addSigner()).addContracts()).build();
        const state = {
          ctx: ctx
        }
        dispatch(studentSignin(state))
        if (await stdLogin(ctx)) {
          router.push('/students');
        } else {
          router.push('/students/signup');
        }

      } catch (err) {
        console.log(err.message);
      }
    }
    else {
      console.log("Metamask not installed.")
    }
  }

  async function adminLogin() {
    if (typeof window != "undefined" && typeof window.ethereum != "undefined") {
      try {
        // Metamask avalable
        // const accounts = await window.ethereum.request({ method: "eth_requestAccounts" })
        // const state = {
        //   address: accounts[0]
        // }
        const ctx = (await (await (await new AstContextBuilder().addProvider()).addSigner()).addContracts()).build();
        const state = {
          ctx: ctx
        }
        dispatch(adminSignin(state))
        if (isAdmin(ctx)) {
          router.push('/admin')
        } else {
          console.log("Unable to login as admin");
        }
      } catch (err) {
        console.log(err.message)
      }
    }
    else {
      console.log("Metamask not installed.")
    }
  }

  return (
    <>
      <div className="container-scroller">
        <div className="container-fluid page-body-wrapper full-page-wrapper">
          <div className="row w-100 m-0">
            <div className="content-wrapper full-page-wrapper d-flex align-items-center auth login-bg">
              <div className="card col-lg-4 mx-auto">
                <div className="card-body px-5 py-5">
                  <form>

                    <div className="d-flex">
                      <button onClick={(e) => {
                        e.preventDefault()
                        router.push('students/signup')
                        studentLogin()
                      }} style={{ "padding": "30px" }} className="btn btn-facebook mr-2 col">Student</button>
                      <button onClick={(e) => {
                        e.preventDefault()
                        adminLogin()
                      }} style={{ "padding": "30px" }} className="btn btn-google col">Admin</button>
                    </div>
                  </form>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

    </>
  )
}
