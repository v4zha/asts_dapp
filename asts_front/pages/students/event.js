import { useState } from "react"
import { useSelector } from "react-redux"
function event() {
  const [token, setToken] = useState("Token")
  const user = useSelector((state) => state.student)
  return (
    <>
      <div className="container-scroller">
        {/* partial:../../partials/_sidebar.html */}
        <nav className="sidebar sidebar-offcanvas" id="sidebar">
          <div className="sidebar-brand-wrapper d-none d-lg-flex align-items-center justify-content-center fixed-top">
            <a className="sidebar-brand brand-logo" href="../../index.html"><img src="../../assets/images/logo.svg" alt="logo" /></a>
            <a className="sidebar-brand brand-logo-mini" href="../../index.html"><img src="../../assets/images/logo-mini.svg" alt="logo" /></a>
          </div>
          <ul className="nav">
            <li className="nav-item profile">
              <div className="profile-desc">
                <div className="profile-pic">
                  <div className="count-indicator">
                    <img className="img-xs rounded-circle" src="../../assets/images/faces/face15.jpg" alt="" />
                    <span className="count bg-success" />
                  </div>
                  <div className="profile-name">
                    <h5 className="mb-0 font-weight-normal">{user.name}</h5>
                  </div>
                </div>
                <a href="#" id="profile-dropdown" data-toggle="dropdown"><i className="mdi mdi-dots-vertical" /></a>
                <div className="dropdown-menu dropdown-menu-right sidebar-dropdown preview-list" aria-labelledby="profile-dropdown">
                  <a href="#" className="dropdown-item preview-item">
                    <div className="preview-thumbnail">
                      <div className="preview-icon bg-dark rounded-circle">
                        <i className="mdi mdi-settings text-primary" />
                      </div>
                    </div>
                    <div className="preview-item-content">
                      <p className="preview-subject ellipsis mb-1 text-small">
                        Account settings
                      </p>
                    </div>
                  </a>
                  <div className="dropdown-divider" />
                  <a href="#" className="dropdown-item preview-item">
                    <div className="preview-thumbnail">
                      <div className="preview-icon bg-dark rounded-circle">
                        <i className="mdi mdi-onepassword text-info" />
                      </div>
                    </div>
                    <div className="preview-item-content">
                      <p className="preview-subject ellipsis mb-1 text-small">
                        Change Password
                      </p>
                    </div>
                  </a>
                  <div className="dropdown-divider" />
                  <a href="#" className="dropdown-item preview-item">
                    <div className="preview-thumbnail">
                      <div className="preview-icon bg-dark rounded-circle">
                        <i className="mdi mdi-calendar-today text-success" />
                      </div>
                    </div>
                    <div className="preview-item-content">
                      <p className="preview-subject ellipsis mb-1 text-small">
                        To-do list
                      </p>
                    </div>
                  </a>
                </div>
              </div>
            </li>
            <li className="nav-item nav-category">
              <span className="nav-link">Navigation</span>
            </li>
            <li className="nav-item menu-items">
              <a className="nav-link" href="../../index.html">
                <span className="menu-icon">
                  <i className="mdi mdi-speedometer" />
                </span>
                <span className="menu-title">Dashboard</span>
              </a>
            </li>
            <li className="nav-item menu-items">
              <a className="nav-link" data-toggle="collapse" href="#ui-basic" aria-expanded="false" aria-controls="ui-basic">
                <span className="menu-icon">
                  <i className="mdi mdi-laptop" />
                </span>
                <span className="menu-title">Basic UI Elements</span>
                <i className="menu-arrow" />
              </a>
              <div className="collapse" id="ui-basic">
                <ul className="nav flex-column sub-menu">
                  <li className="nav-item">
                    <a className="nav-link" href="../../pages/ui-features/buttons.html">Buttons</a>
                  </li>
                  <li className="nav-item">
                    <a className="nav-link" href="../../pages/ui-features/dropdowns.html">Dropdowns</a>
                  </li>
                  <li className="nav-item">
                    <a className="nav-link" href="../../pages/ui-features/typography.html">Typography</a>
                  </li>
                </ul>
              </div>
            </li>
            <li className="nav-item menu-items">
              <a className="nav-link" href="../../pages/forms/basic_elements.html">
                <span className="menu-icon">
                  <i className="mdi mdi-playlist-play" />
                </span>
                <span className="menu-title">Form Elements</span>
              </a>
            </li>
            <li className="nav-item menu-items">
              <a className="nav-link" href="../../pages/tables/basic-table.html">
                <span className="menu-icon">
                  <i className="mdi mdi-table-large" />
                </span>
                <span className="menu-title">Tables</span>
              </a>
            </li>
            <li className="nav-item menu-items">
              <a className="nav-link" href="../../pages/charts/chartjs.html">
                <span className="menu-icon">
                  <i className="mdi mdi-chart-bar" />
                </span>
                <span className="menu-title">Charts</span>
              </a>
            </li>
            <li className="nav-item menu-items">
              <a className="nav-link" href="../../pages/icons/mdi.html">
                <span className="menu-icon">
                  <i className="mdi mdi-contacts" />
                </span>
                <span className="menu-title">Icons</span>
              </a>
            </li>
            <li className="nav-item menu-items">
              <a className="nav-link" data-toggle="collapse" href="#auth" aria-expanded="false" aria-controls="auth">
                <span className="menu-icon">
                  <i className="mdi mdi-security" />
                </span>
                <span className="menu-title">User Pages</span>
                <i className="menu-arrow" />
              </a>
              <div className="collapse" id="auth">
                <ul className="nav flex-column sub-menu">
                  <li className="nav-item">
                    <a className="nav-link" href="../../pages/samples/blank-page.html">
                      Blank Page
                    </a>
                  </li>
                  <li className="nav-item">
                    <a className="nav-link" href="../../pages/samples/error-404.html">
                      404
                    </a>
                  </li>
                  <li className="nav-item">
                    <a className="nav-link" href="../../pages/samples/error-500.html">
                      500
                    </a>
                  </li>
                  <li className="nav-item">
                    <a className="nav-link" href="../../pages/samples/login.html">
                      Login
                    </a>
                  </li>
                  <li className="nav-item">
                    <a className="nav-link" href="../../pages/samples/register.html">
                      Register
                    </a>
                  </li>
                </ul>
              </div>
            </li>
            <li className="nav-item menu-items">
              <a className="nav-link" href="http://www.bootstrapdash.com/demo/corona-free/jquery/documentation/documentation.html">
                <span className="menu-icon">
                  <i className="mdi mdi-file-document-box" />
                </span>
                <span className="menu-title">Documentation</span>
              </a>
            </li>
          </ul>
        </nav>
        {/* partial */}
        <div className="container-fluid page-body-wrapper">
          {/* partial:../../partials/_navbar.html */}
          <nav className="navbar p-0 fixed-top d-flex flex-row">
            <div className="navbar-brand-wrapper d-flex d-lg-none align-items-center justify-content-center">
              <a className="navbar-brand brand-logo-mini" href="index.html"><img src="assets/images/logo-mini.svg" alt="logo" /></a>
            </div>
            <div className="navbar-menu-wrapper flex-grow d-flex align-items-stretch">
              <ul className="navbar-nav navbar-nav-right">
                <li className="nav-item dropdown d-none d-lg-block">
                  <div className="nav-link btn btn-success create-new-button" id="createbuttonDropdown" data-toggle="dropdown" aria-expanded="false" onClick={() => router.push('/students/event')}>+ Register New Event</div>
                </li>
              </ul>
              <button className="navbar-toggler navbar-toggler-right d-lg-none align-self-center" type="button" data-toggle="offcanvas">
                <span className="mdi mdi-format-line-spacing" />
              </button>
            </div>
          </nav>
          {/* partial */}
          <div className="main-panel">
            <div className="content-wrapper">
              <div className="row">
                <div className="col-md-6 grid-margin stretch-card">
                  <div className="card">
                    <div className="card-body">
                      <h4 className="card-title">Register Event</h4>
                      {/* <p className="card-description">Basic bootstrap input groups</p> */}
                      <div className="form-group">
                        <div className="input-group">
                          <input type="text" className="form-control" placeholder="Event Name" aria-label="Username" aria-describedby="basic-addon1" />
                        </div>
                      </div>
                      <div className="form-group">
                        <div className="input-group">
                          <div className="input-group-prepend">
                            <button className="btn btn-sm btn-outline-primary dropdown-toggle" type="button" data-toggle="dropdown" aria-haspopup="true" aria-expanded="false">
                              {token}
                            </button>
                            <div className="dropdown-menu">
                              <div onClick={() => { setToken("Social") }} className="dropdown-item" style={{ "cursor": "pointer" }}>Social</div>
                              <div onClick={() => { setToken("Academic") }} className="dropdown-item" style={{ "cursor": "pointer" }}>Academic</div>
                              <div onClick={() => { setToken("Sports") }} className="dropdown-item" style={{ "cursor": "pointer" }}>Sports</div>
                            </div>
                          </div>
                          <input placeholder="Amount" type="text" className="form-control" aria-label="Text input with dropdown button" />
                        </div>
                      </div>
                      <div className="form-group">
                        <div className="input-group">
                          <input type="text" className="form-control" placeholder="Stake Amount" aria-label="Username" aria-describedby="basic-addon1" />
                        </div>
                      </div>
                      <div className="form-group">
                        <label htmlFor="exampleTextarea1">Description</label>
                        <textarea className="form-control" id="exampleTextarea1" rows={4} defaultValue={""} />
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </div>
            {/* content-wrapper ends */}
            {/* partial:../../partials/_footer.html */}
            <footer className="footer">
              <div className="d-sm-flex justify-content-center justify-content-sm-between">
                <span className="text-muted d-block text-center text-sm-left d-sm-inline-block">Copyright © bootstrapdash.com 2020</span>
                <span className="float-none float-sm-right d-block mt-1 mt-sm-0 text-center">
                  Free
                  <a href="https://www.bootstrapdash.com/bootstrap-admin-template/" target="_blank">Bootstrap admin templates</a>
                  from Bootstrapdash.com</span>
              </div>
            </footer>
            {/* partial */}
          </div>
          {/* main-panel ends */}
        </div>
        {/* page-body-wrapper ends */}
      </div>

    </>
  )
}

export default event
