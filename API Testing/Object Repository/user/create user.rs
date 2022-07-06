<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>create user</name>
   <tag></tag>
   <elementGuidId>cd1340e5-7b20-4c2e-84c7-eeee37be680b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;email\&quot; : \&quot;JakeJohn@email.com\&quot;,\n    \&quot;username\&quot;:\&quot;Jake\&quot;,\n    \&quot;password\&quot; : \&quot;tes123\&quot;,    \n    \&quot;role\&quot; : \&quot;user\&quot;,\n    \&quot;nik\&quot;: \&quot;1231313190\&quot;,\n    \&quot;noHp\&quot;: \&quot;0812312\&quot;,\n    \&quot;nama\&quot;: \&quot;Shawn\&quot;,\n    \&quot;gender\&quot;: \&quot;male\&quot;,\n    \&quot;tglLahir\&quot;: \&quot;2000-05-28\&quot;,\n    \&quot;image\&quot;: \&quot;photos.jpg\&quot;,\n    \&quot;address\&quot;: \&quot;JL Bintang\&quot;,\n    \&quot;kota\&quot;: \&quot;Batam\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>bb600c5f-0974-42b4-adc2-fd415b3bafa6</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://booking-vaksin-alta.herokuapp.com/api/user</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>2b9c8337-645b-4529-9dac-a40d112c841a</id>
      <masked>false</masked>
      <name>variable</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
