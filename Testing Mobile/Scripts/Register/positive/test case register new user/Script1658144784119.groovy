import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

Mobile.startApplication('C:\\\\Users\\\\A\\\\Downloads\\\\app-release (3).apk', false)

Mobile.tap(findTestObject('Register/positive/button daftar'), 0)

Mobile.tap(findTestObject('Login/positive/input email'), 0)

Mobile.setText(findTestObject('Login/positive/input email'), 'nicholas@gmail.com', 0)

Mobile.tap(findTestObject('Login/positive/input password'), 0)

Mobile.setText(findTestObject('Login/positive/input password'), 'Frans1234', 0)

Mobile.pressBack()

Mobile.tap(findTestObject('Register/positive/button daftar'), 0)

Mobile.delay(3, FailureHandling.STOP_ON_FAILURE)

Mobile.tap(findTestObject('Register/positive/button icon foto'), 0)

Mobile.delay(3, FailureHandling.STOP_ON_FAILURE)

Mobile.tap(findTestObject('Register/positive/image'), 0)

Mobile.delay(3, FailureHandling.STOP_ON_FAILURE)

Mobile.tap(findTestObject('Register/positive/nama lengkap'), 0)

Mobile.setText(findTestObject('Register/positive/nama lengkap'), 'Purnomo', 0)

Mobile.delay(3, FailureHandling.STOP_ON_FAILURE)

Mobile.tap(findTestObject('Register/positive/nik'), 0)

Mobile.setText(findTestObject('Register/positive/nik'), '3319087890985690', 0)

Mobile.delay(3, FailureHandling.STOP_ON_FAILURE)

Mobile.tap(findTestObject('Register/positive/tanggal lahir'), 0)

Mobile.tap(findTestObject('Register/positive/icon kalender'), 0)

Mobile.setText(findTestObject('Register/positive/input date'), '09/09/1998', 0)

Mobile.tap(findTestObject('Register/positive/ok'), 0)

Mobile.delay(3, FailureHandling.STOP_ON_FAILURE)

Mobile.tap(findTestObject('Register/positive/no hp'), 0)

Mobile.setText(findTestObject('Register/positive/no hp'), '089098909890', 0)

Mobile.delay(3, FailureHandling.STOP_ON_FAILURE)

Mobile.tap(findTestObject('Register/positive/jenis kelamin'), 0)

Mobile.tap(findTestObject('Register/positive/laki2'), 0)

Mobile.delay(3, FailureHandling.STOP_ON_FAILURE)

Mobile.tap(findTestObject('Register/positive/simpan'), 0)

Mobile.delay(5, FailureHandling.STOP_ON_FAILURE)

Mobile.closeApplication()

