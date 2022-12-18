// SPDX-License-Identifier: MIT
pragma abicoder v2;
pragma solidity >=0.7.0 <0.9.0;

import {AsToken} from "./AsToken.sol";
import {ScToken} from "./ScToken.sol";
import {SpToken} from "./SpToken.sol";

contract Student {
    AsToken stAcadamic;
    SpToken stSports;
    ScToken stSocial;
    enum Department {
        Mech,
        EEE,
        Civil,
        EC,
        CS,
        IT
    }
    address public owner;
    //Students made public for testing
    mapping(address => StudentData) public Students;
    struct Tokens {
        AsToken acadamic;
        SpToken sports;
        ScToken social;
    }
    struct StudentData {
        string name;
        Department department;
        uint32 admnNo;
        Tokens tokens;
        bool isStd;
    }
    struct StudentInfo {
        string name;
        Department department;
        uint32 admnNo;
        TokenCnt tokens;
        bool isStd;
    }
    struct TokenCnt {
        uint32 acadamic;
        uint32 sports;
        uint32 social;
    }
    enum TokenKind {
        Acadamic,
        Sports,
        Social
    }

    constructor() {
        owner = msg.sender;
        stAcadamic = new AsToken();
        stSports = new SpToken();
        stSocial = new ScToken();
    }

    modifier onlyAdmin(address send) {
        require(
            send == owner,
            "This operation cann only be performed by administrator"
        );
        _;
    }

    modifier isStudent(address stdAddr) {
        require(
            Students[stdAddr].isStd == true,
            "This operation can only be perfomed by registered Students"
        );
        _;
    }

    event StudentRegistered(string name, uint32 admnNo, Department dep);

    function setStdStatus(address stdAddr, bool status) external {
        Students[stdAddr].isStd = status;
    }

    function getStdStatus(address stdAddr) external view returns (bool) {
        return Students[stdAddr].isStd;
    }

    function registerStudent(
        string memory name,
        uint32 admnNo,
        Department department
    ) external returns (bool) {
        StudentData memory std = StudentData({
            name: name,
            department: department,
            admnNo: admnNo,
            tokens: _setInitToken(),
            isStd: true
        });
        Students[msg.sender] = std;
        emit StudentRegistered(name, admnNo, department);
        return true;
    }

    function _setInitToken() internal view returns (Tokens memory) {
        return
            Tokens({acadamic: stAcadamic, social: stSocial, sports: stSports});
    }

    function addToken(
        address toStd,
        uint32 amt,
        TokenKind kind,
        address own
    ) external onlyAdmin(own) isStudent(toStd) returns (bool) {
        StudentData memory std = Students[toStd];
        if (kind == TokenKind.Acadamic) {
            std.tokens.acadamic.mint(toStd, amt);
        } else if (kind == TokenKind.Social) {
            std.tokens.social.mint(toStd, amt);
        } else {
            std.tokens.sports.mint(toStd, amt);
        }
        return true;
    }

    function getOwner() external view returns (address) {
        return owner;
    }

    function getStudent() external view returns (StudentInfo memory) {
        StudentData memory std = Students[msg.sender];
        StudentInfo memory stdInfo = StudentInfo({
            name: std.name,
            department: std.department,
            admnNo: std.admnNo,
            tokens: getAllTokens(msg.sender),
            isStd: std.isStd
        });
        return stdInfo;
    }

    function getAcToken(address stdAddr)
        public
        view
        isStudent(stdAddr)
        returns (uint32)
    {
        return uint32(Students[stdAddr].tokens.acadamic.balanceOf(stdAddr));
    }

    function isOwner() external view returns (bool) {
        return msg.sender == owner;
    }

    function getScToken(address stdAddr)
        public
        view
        isStudent(stdAddr)
        returns (uint32)
    {
        return uint32(Students[stdAddr].tokens.social.balanceOf(stdAddr));
    }

    function getSpToken(address stdAddr)
        public
        view
        isStudent(stdAddr)
        returns (uint32)
    {
        return uint32(Students[stdAddr].tokens.sports.balanceOf(stdAddr));
    }

    function getAllTokens(address stdAddr)
        public
        view
        isStudent(stdAddr)
        returns (TokenCnt memory)
    {
        return
            TokenCnt({
                acadamic: getAcToken(stdAddr),
                social: getScToken(stdAddr),
                sports: getSpToken(stdAddr)
            });
    }

    function getTokenAmt(address stdAddr, TokenKind kind)
        external
        view
        returns (uint32)
    {
        if (kind == TokenKind.Acadamic) {
            return getAcToken(stdAddr);
        } else if (kind == TokenKind.Social) {
            return getScToken(stdAddr);
        } else {
            return getSpToken(stdAddr);
        }
    }
}
