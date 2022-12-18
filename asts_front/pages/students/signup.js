import { useState } from 'react'
import { studentSignin } from '../../Slices/loginSlice'
import { useRouter } from 'next/router'
import { regStudent } from "../../std_api/src/student"
import { useDispatch, useSelector } from "react-redux"

function signup() {
  const dispatch = useDispatch()
  const router = useRouter()

  const [name, setName] = useState("")
  const [dept, setDept] = useState("")
  const [adm, setAdm] = useState("")

  const ctx = useSelector((state) => state.auth.ctx)

  const handleName = (e) => {
    setName(e.target.value)
  }

  const handleDept = (e) => {
    setDept(e.target.value)
  }

  const handleAdm = (e) => {
    setAdm(e.target.value)
  }

  const handleRegister = async () => {
    const state = {
      address: ctx.address,
      name: name,
      adm: adm,
      dept: dept
    }
    dispatch(studentSignin(state))
    await regStudent(name, adm, dept, ctx);
    router.push('/students')
  }

  return (
    <>
      <div className="container-scroller">
        <div className="container-fluid page-body-wrapper full-page-wrapper">
          <div className="row w-100 m-0">
            <div className="content-wrapper full-page-wrapper d-flex align-items-center auth login-bg">
              <div className="card col-lg-4 mx-auto">
                <div className="card-body px-5 py-5">
                  <h3 className="card-title text-left mb-3">Register</h3>
                  <form>
                    <div className="form-group">
                      <label>Name</label>
                      <input onChange={(e) => { handleName(e) }} type="text" className="form-control p_input" />
                    </div>
                    <div className="form-group">
                      <label>Department</label>
                      <input type="Number" onChange={(e) => { handleDept(e) }} className="form-control p_input" />
                    </div>
                    <div className="form-group">
                      <label>Admission Number</label>
                      <input type="Number" onChange={(e) => { handleAdm(e) }} className="form-control p_input" />
                    </div>
                    {/* <div className="form-group d-flex align-items-center justify-content-between">
                                            <div className="form-check">
                                                <label className="form-check-label">
                                                    <input type="checkbox" className="form-check-input" /> Remember me </label>
                                            </div>
                                            <a href="#" className="forgot-pass">Forgot password</a>
                                        </div> */}
                    <div className="text-center">
                      <button onClick={(e) => { e.preventDefault(); handleRegister() }} type="submit" className="btn btn-primary btn-block enter-btn">Login</button>
                    </div>
                    {/* <div className="d-flex">
                                            <button className="btn btn-facebook col mr-2">
                                                <i className="mdi mdi-facebook" /> Facebook </button>
                                            <button className="btn btn-google col">
                                                <i className="mdi mdi-google-plus" /> Google plus </button>
                                        </div>
                                        <p className="sign-up text-center">Already have an Account?<a href="#"> Sign Up</a></p>
                                        <p className="terms">By creating an account you are accepting our<a href="#"> Terms &amp; Conditions</a></p> */}
                  </form>
                </div>
              </div>
            </div>
            {/* content-wrapper ends */}
          </div>
          {/* row ends */}
        </div>
        {/* page-body-wrapper ends */}
      </div>

    </>

  )
}

export default signup
