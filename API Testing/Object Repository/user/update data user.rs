<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>update data user</name>
   <tag></tag>
   <elementGuidId>9c15af24-63cc-489b-8c29-a5977fdf0cc1</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\&quot;email\&quot;: \&quot;ica@gmail.com\&quot;,\n    \&quot;username\&quot;: \&quot;ica\&quot;,\n    \&quot;password\&quot;: \&quot;tes123\&quot;,\n    \&quot;role\&quot;: \&quot;user\&quot;,\n    \&quot;nik\&quot;: \&quot;29358023131\&quot;,\n    \&quot;noHp\&quot;: \&quot;0840332012413\&quot;,\n    \&quot;nama\&quot;: \&quot;ica sevina\&quot;,\n    \&quot;gender\&quot;: \&quot;female\&quot;,\n    \&quot;tglLahir\&quot;: \&quot;2001-05-27T00:00:00.000+00:00\&quot;,\n    \&quot;image\&quot;: \&quot;https://images.unsplash.com/photo-1573496359142-b8d87734a5a2?ixlib\u003drb-1.2.1\u0026ixid\u003dMnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8\u0026auto\u003dformat\u0026fit\u003dcrop\u0026w\u003d388\u0026q\u003d80\&quot;,\n    \&quot;address\&quot;: \&quot;jln unta\&quot;,\n    \&quot;kota\&quot;: \&quot;kalimantan\&quot;\n}&quot;,
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
      <webElementGuid>a038db7a-6547-4597-88eb-52b9cf5a4d50</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>https://booking-vaksin-alta.herokuapp.com/api/user/124</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
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
