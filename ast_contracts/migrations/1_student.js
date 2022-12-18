const Student = artifacts.require("Student");
const StdEvents = artifacts.require("ASTEvent");
module.exports = (deployer) => {
  deployer.deploy(Student).then(
    () => {
      return deployer.deploy(StdEvents, Student.address);
    }
  );
};
